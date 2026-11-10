import json,glob,re,sys,os
GH=os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','notes','gh')
pat=re.compile(sys.argv[2],re.I); r=sys.argv[1]
rel=[]
for f in sorted(glob.glob(f'{GH}/{r}_releases_p*.json')): rel+=json.load(open(f))
rel.sort(key=lambda x:x['published_at'])
seen={}
for x in rel:
    for line in (x.get('body') or '').split('\n'):
        m=re.match(r'\* (.*) by @(\S+) in (https://\S+/pull/(\d+))',line.strip())
        if m and m.group(4) not in seen:
            seen[m.group(4)]=(x['tag_name'],x['published_at'][:10],m.group(1),m.group(3))
for k,(t,d,ti,u) in seen.items():
    if pat.search(ti): print(d,t,'|',ti,'|',u)
print('TOTAL releases',len(rel),'first',rel[0]['tag_name'],rel[0]['published_at'][:10],'last',rel[-1]['tag_name'], 'distinct PRs',len(seen),file=sys.stderr)
