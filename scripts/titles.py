import sys; import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import load
it=load(); repo,state,kind=sys.argv[1:4]
for (r,n),x in sorted(it.items(),key=lambda k:-k[0][1]):
    if r!=repo or x['state']!=state: continue
    if x['state']=='closed' and x['closed_at']<'2025-11-10': continue
    if (kind=='PR')!=x['is_pr']: continue
    lb=','.join(l['name'] for l in x['labels'])
    print(n,x['created_at'][:10],(x['closed_at'] or '')[:10],x['title'][:110],('['+lb+']' if lb else ''))
