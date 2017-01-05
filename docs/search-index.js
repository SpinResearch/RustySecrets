var searchIndex = {};
searchIndex["rusty_secrets"] = {"doc":"`RustySecrets` implements Shamir&#39;s secret sharing in Rust. It provides the possibility to sign shares.","items":[[3,"RustyError","rusty_secrets","Error struct used for generating an `io::Error` from a generic description.",null,null],[11,"fmt","","",0,null],[11,"with_type","","Returns a `RustyError` with a given `RustyErrorType`.",0,{"inputs":[{"name":"rustyerrortypes"}],"output":{"name":"rustyerror"}}],[11,"share_index","","Returns the index of the share that raised the error, if any.",0,null],[11,"share_groups","","Returns the group of shares that were generated during the same secret share.\nIt can be used to provide a debug message to the user telling him what shares are incompatible.",0,null],[11,"fmt","","",0,null],[11,"description","","",0,null],[11,"cause","","",0,null],[11,"from","","",0,{"inputs":[{"name":"error"}],"output":{"name":"rustyerror"}}],[0,"sss","","SSS provides Shamir&#39;s secret sharing with raw data.",null,null],[5,"generate_shares","rusty_secrets::sss","Performs threshold k-out-of-n Shamir&#39;s secret sharing.",null,null],[5,"recover_secret","","Recovers the secret from a k-out-of-n Shamir&#39;s secret sharing.",null,{"inputs":[{"name":"vec"},{"name":"bool"}],"output":{"name":"result"}}],[0,"wrapped_secrets","rusty_secrets","(Beta) `wrapped_secrets` provides Shamir&#39;s secret sharing with a wrapped secret. It currently offers versioning and MIME information about the data.",null,null],[5,"generate_shares","rusty_secrets::wrapped_secrets","Performs threshold k-out-of-n Shamir&#39;s secret sharing.",null,null],[5,"recover_secret","","Recovers the secret from a k-out-of-n Shamir&#39;s secret sharing.",null,{"inputs":[{"name":"vec"},{"name":"bool"}],"output":{"name":"result"}}]],"paths":[[3,"RustyError"]]};
initSearch(searchIndex);
