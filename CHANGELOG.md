2015-02-03:

* I changed the CRC-24 checksum computation to include the coding parameter
  K and the share number N so that these numbers are also protected.
  If you have older shares generated with a previous version, you can still
  decode the secret by simply removing the checksum part of the shares.
* The README now includes more information about the inner workings of
  secretshare and also a note on "perfect secrecy".
