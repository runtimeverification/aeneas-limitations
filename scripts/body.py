import sys; import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import load
it=load(); repo=sys.argv[1]; lim=int(sys.argv[2])
for n in sys.argv[3:]:
    x=it.get((repo,int(n)))
    if not x: print('MISSING',n); continue
    print(f"===== {repo}#{n} [{x['state']}] {x['title']} created {x['created_at'][:10]} closed {(x['closed_at'] or '')[:10]} comments={x['comments']} merged={x.get('pull_request',{}).get('merged_at')}")
    print((x['body'] or '')[:lim])
