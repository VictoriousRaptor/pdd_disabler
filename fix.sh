sh debug.sh
if [ "$?" != "0" ]; then
    echo "退出"
    exit 1
fi
git add  .
git commit -m "prev fix"
cargo clippy --fix
git add  .
git commit -m "fix finished"
