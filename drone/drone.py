# DeCeFi Drone service
# 1. Watches for new orders in blockchain storage loading new hashes
# 2. Waits for an order executed querying to CCAI API with order hash
# 3. Sends instruction to Solana program on an order executed

# Improvement points
# 1. Receive for new orders only instead of periodical RPC calls to read
#    whole the list of orders
# 2. Forget about old (executed) orders

from enum import Enum, unique, auto
from typing import List
import asyncio
import logging
import os
import random
import shelve
import time
import json

import gql
import gql.transport.aiohttp
import solana
import solana.account
import solana.rpc.api


PROCEDURE_PERIOD = 1.0
IDLE_LOGGING_PERIOD = 10.0  # sec
SHELVE_NAME = 'orders_processed.shelve'
CCAI_ENDPOINT = os.environ.get('CCAI_API_ENDPOINT')
SOLANA_ENDPOINT = os.environ.get('SOLANA_RPC_ENDPOINT')
ID_PATH = './id-defeci-drone.json'
DECEFI_PROGRAM_ID = os.environ.get('DECEFI_PROGRAM_ID')


log = logging.getLogger(__name__)
log.setLevel(getattr(logging, os.environ.get('LOGLEVEL', 'INFO').upper()))
log.addHandler(logging.StreamHandler())
# log.basicConfig(level=logging, os.environ.get('LOGLEVEL', 'INFO').upper())
shelve_lock = asyncio.Lock()
solana_client = solana.rpc.api.Client(SOLANA_ENDPOINT)
ccai_gql_transport = gql.transport.aiohttp.AIOHTTPTransport(url=CCAI_ENDPOINT)
with open(ID_PATH, 'r') as secret_key:
    secrent_key_data = bytearray(json.load(secret_key))
solana_acc = solana.rpc.api.Account(secrent_key_data)


class DeCeFiSC:
    def __init__(self, connection: solana.rpc.api.Client, pubkey, program_id):
        self.connection = connection
        self.acc_pubkey = pubkey
        self.program_id = program_id

    async def getOrdersHashes(self):
        log.debug('getOrdersHashes')
        # let's mock it, one can add orders manually appending to the file
        with open('./orders_hashes.txt', 'r') as fd:
            hashes: List[str] = fd.readlines()
        return hashes

    async def unlockFunds(self, order_hash):
        log.debug('unlockFunds')
        log.info('--- UNLOCK FUNDS ---')  # printing to let operator do it manually
        log.info(f'>>> ORDER HASH: "{order_hash}" <<<')
        log.info('---')


decefi_sc = DeCeFiSC(solana_client, solana_acc.pubkey(), DECEFI_PROGRAM_ID)


class OrderStatus(Enum):
    NOT_INTERESTING = auto()
    EXECUTED = auto()


async def get_order_status(session, order_hash: str) -> OrderStatus:
    '''Queries CCAI API for a status of order given.

    CCAI Terminal API retuns one of the possible values:
        waitforentry
        trailingentry
        inentry
        takeprofit
        stoploss
        end
        cancelled  # returns it on status not set as well
        error
    '''
    log.info(f'Querying CCAI API for order {order_hash} status')
    status: OrderStatus = None
    getOrderStatusQuery = gql.gql('''
        query {
          getStrategyStatusByHash(
            input: { hash: "%s" }
          )
        }
    ''' % order_hash)  # using formatted text instead of GQL var

    result = await session.execute(getOrderStatusQuery)
    try:
        read_status = result['getStrategyStatusByHash'].lower()
    except KeyError as e:  # supress query errors
        log.warn(f'Exception while querying for CCAI API: {e}')
        return OrderStatus.NOT_INTERESTING
    log.debug(f'Status {result=}, {order_hash=}')
    if read_status in ['end']:
        status = OrderStatus.EXECUTED
    else:
        status = OrderStatus.NOT_INTERESTING
    return status


def get_orders_hashes() -> List[str]:
    '''Queries Solana RPC for DeCeFi contract orders hashes.'''
    log.info(f'Querying Solana RPC for orders')
    # Call to solana
    hashes: List[str] =
    return hashes


async def notify_order_executed(order_hash: str, shelve_name) -> None:
    '''Notifies DeCeFi contract on order executed and stores hash.'''
    log.info(f'Notifying DeCeFi contract on order {order_hash} executed')
    decefi_sc.unlockFunds(order_hash)
    async with shelve_lock:
        with shelve.open(shelve_name) as db:
            # append to orders list in the shelve
            items = db['processed_orders_hashes']
            items.append(order_hash)
            db['processed_orders_hashes'] = items


async def main():
    last_call_time = time.time()
    try:
        while True:
            # 1. Read all the orders from blockchain
            all_orders_hashes = get_orders_hashes()

            # 2. Read all the orders processed from local storage
            with shelve.open(SHELVE_NAME) as db:
                try:
                    processed_orders_hashes = db['processed_orders_hashes']
                except KeyError:
                    log.info('No data on orders processed, '
                             'new storage will be created')
                    db['processed_orders_hashes'] = []
                    processed_orders_hashes = []

            log.debug('processed_orders_hashes')
            log.debug(processed_orders_hashes)

            log.debug('all_orders_hashes')
            log.debug(all_orders_hashes)

            # 3. Remove processed or not executed
            new_orders_hashes = [h for h in all_orders_hashes
                                 if h not in processed_orders_hashes]

            log.debug('new_orders_hashes')
            log.debug(new_orders_hashes)

            log.info(f'{len(all_orders_hashes)} orders, '
                     f'{len(all_orders_hashes) - len(new_orders_hashes)} processed earlier, '
                     f'{len(new_orders_hashes)} new')

            async with gql.Client(
                transport=ccai_gql_transport, fetch_schema_from_transport=True,
            ) as session:
                new_orders_statuses = await asyncio.gather(*[
                    get_order_status(session, h) for h in new_orders_hashes
            ])
            new_executed_orders_hashes = []
            for h, cond in zip(new_orders_hashes, new_orders_statuses):
                if cond == OrderStatus.EXECUTED:
                    new_executed_orders_hashes.append(h)

            log.debug(f'{new_executed_orders_hashes=}')

            if new_executed_orders_hashes:
                log.debug(f'New orders executed: {new_executed_orders_hashes}')
                last_call_time = time.time()
            elif (dt := time.time() - last_call_time) > IDLE_LOGGING_PERIOD:
                log.debug(f'No new orders last {dt} seconds')
                last_call_time = time.time()

            # 4. Call instruction for all the rest and save the result in storage
            await asyncio.gather(*[
                notify_order_executed(h, SHELVE_NAME) for h in new_executed_orders_hashes
            ])

            await asyncio.sleep(PROCEDURE_PERIOD)
    except Exception as e:
        log.error(f'Exception: {e}')


if __name__ == '__main__':
    log.info('Starting drone service...')
    log.info('Configuration: '
             f'{PROCEDURE_PERIOD=}, {IDLE_LOGGING_PERIOD=}, {SHELVE_NAME=}, '
             f'PUBKEY={solana_acc.public_key()}')
    asyncio.run(main())
    log.info('Shutdown...')
