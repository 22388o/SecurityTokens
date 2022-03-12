lncli addinvoice --amt <amount in satoshis> --memo=’my first amp’ --amp
lncli payinvoice --pay_req <the amp invoice created by the receiver>
lncli sendpayment --amt <amount in satoshis> --dest <public key of receiver> --amp
lncli payinvoice --pay_req <the amp invoice created by the receiver> --pay_addr <the sha256 hash of a random number>
lncli payinvoice --pay_req <the amp invoice created by the receiver> --amp-reuse
  
