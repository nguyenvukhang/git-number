# unstaged modified file (number)

init 3
echo "_" >file_3
gitnu status
gitnu add 3 # use number on file_3
gitnu commit -m "first"
echo "_" >>file_3
gitnu status
gitnu add 1 # use number on file_3 again
save gitnu status

# --------------------------------------------------------------------
# On branch main
# Changes to be committed:
# 1	[32mnew file:   file_0001[m
# 
# Untracked files:
# 2	[31mfile_0002[m
# 3	[31mfile_3[m
# 
