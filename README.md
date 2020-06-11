# checkasum

`checkasum` is a simple tool for hashing a file, and comparing the result with a known 'checksum' value.
This allows the user to confirm the integrity of a file downloaded from the internet. 
If the value returned from hashing a file matches the given checksum, 
then chances are that the file has not been tampered with.

## Examples

- `checkasum -m sha256 -p /path/to/file -e somechecksum`
- `checkasum --method sha256 --path /path/to/file --expected somechecksum`