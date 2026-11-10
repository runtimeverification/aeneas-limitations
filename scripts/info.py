import sys; import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import load
it=load()
for a in sys.argv[1:]:
    r,n=a.split('#'); x=it.get((r,int(n)))
    if not x: print(a,'NOT IN DATASET'); continue
    m=x.get('pull_request',{}).get('merged_at') if x['is_pr'] else None
    print(a,'PR' if x['is_pr'] else 'issue',x['state'],x['created_at'][:10],(x['closed_at'] or '')[:10],'merged '+m[:10] if m else '',x['title'][:110])
