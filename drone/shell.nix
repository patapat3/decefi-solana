with import <nixpkgs> {};

mkShell {
  buildInputs = [
    # db conn
    kubectl lsof awscli2 mongodb-compass
  ];
}
