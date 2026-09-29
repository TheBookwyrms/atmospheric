# use PowerShell instead of sh on windows:
set windows-shell  := ["C:/Program Files/Git/git-bash.exe", "-c"]


alias p := push
push msg:
    git add .
    git commit -m "{{msg}}"
    git push