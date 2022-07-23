resim reset
OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export a1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

PK_OP=$(resim publish ".")
echo $PK_OP
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo $PACKAGE

CP1_OP=$(resim run "./transactions/instantiate_component1.rtm")
echo $CP1_OP
export COMPONENT1=$(echo "$CP1_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
echo $COMPONENT1

CP2_OP=$(resim run "./transactions/instantiate_component2.rtm")
echo $CP2_OP
export COMPONENT2=$(echo "$CP2_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
echo $COMPONENT2