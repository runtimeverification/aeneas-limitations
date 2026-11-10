import sys; import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import load
it=load()
from collections import Counter
c=Counter()
for (r,n),x in it.items():
    if x['state']=='closed' and x['closed_at']<'2025-11-10': continue
    c[(r,x['state'],'PR' if x['is_pr'] else 'issue')]+=1
for k,v in sorted(c.items()): print(k,v)
