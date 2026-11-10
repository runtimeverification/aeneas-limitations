import json,glob,sys,os
GH=os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','notes','gh')
for r in ['aeneas','charon']:
    rel=[]
    for f in sorted(glob.glob(f'{GH}/{r}_releases_p*.json')): rel+=json.load(open(f))
    print(r,'releases',len(rel),'newest',rel[0]['tag_name'],rel[0]['published_at'],'oldest',rel[-1]['tag_name'],rel[-1]['published_at'])
    bl=[x for x in rel if (x.get('body') or '').strip()]
    print(' with body:',len(bl)); 
    for x in rel[:5]: print('  ',x['tag_name'],x['published_at'],repr((x.get('body') or '')[:300]))
