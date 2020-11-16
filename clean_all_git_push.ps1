cd base
cargo clean
cd ..
cd shape
cargo clean
cd ..
git add *
git commit -m $(Get-Date)
git push origin master