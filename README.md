
```bash
asq [-v] IP-ADDRESS
```

Command line command to tell AS of an IP address.
AS, Autonomous System is the top level of the IP
routing in the Internet.
AS usually means the international operator of the IP address.

# Example

(at command shell)
```bash
$ host -4 -t A ibm.com
ibm.com has address 129.42.38.10
$ asq 129.42.38.10
IBM-EI
$ asq -v 129.42.38.10
IBM-EI - IBM - Events Infrastructure - US
IBMCCH-RTP - IBM - US
ISSC-AS - IBM Corporation - US
$ host -4 -t A www.ibm.com
...
... has address 59.151.164.181
$ asq 59.151.164.181
AKAMAI-AS
$ asq -v 59.151.164.181
AKAMAI-AS - Akamai Technologies, Inc. - US
AKAMAI-TYO-AP - Akamai Technologies Tokyo ASN - SG
```

# Installation

```cargo install --path .```

installs the asq command to ~/.cargo/bin
