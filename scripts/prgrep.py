import sys,re; import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import load
it=load(); repo=sys.argv[1]; pat=re.compile(sys.argv[2],re.I)
for (r,n),x in sorted(it.items(),key=lambda k:-k[0][1]):
    if r!=repo or not x['is_pr']: continue
    if x['state']=='closed' and x['closed_at']<'2025-11-10': continue
    if not pat.search(x['title']): continue
    merged=(x['pull_request'].get('merged_at') or '')[:10]
    st='merged '+merged if merged else ('open' if x['state']=='open' else 'closed-unmerged '+x['closed_at'][:10])
    print(n,x['created_at'][:10],st,x['title'][:120])
