# staged file (number)

init 2
touch file:gold
gitnu status
gitnu add 1-2 # use number
gitnu status
gitnu reset 1 # use number again, on file:gold
save gitnu status

# --------------------------------------------------------------------
# On branch main
#
# No commits yet
#
# Changes to be committed:
# 1	[32mnew file:   file_0001[m
#
# Untracked files:
# 2	[31mfile:gold[m
# 3	[31mfile_0002[m
#
