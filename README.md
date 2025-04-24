# solana_60days
solana demos are written based on the anchor framework(https://www.rareskills.io/solana-tutorial）

## how to merge a sub repo to main repo
* git remote add day_2-remote ../day_2
* git subtree add --prefix=day_2  day_2-remote master --squash
* git commit -m ""
* git push origin main
