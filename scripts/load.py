import json,glob,re,sys,os
GH=os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','notes','gh')
def load():
    items={}
    for r in ['aeneas','charon']:
        for f in sorted(glob.glob(f'{GH}/{r}_open*_p*.json'))+sorted(glob.glob(f'{GH}/{r}_closed_p*.json')):
            for x in json.load(open(f)):
                x['repo']=r; x['is_pr']='pull_request' in x
                items[(r,x['number'])]=x
    return items
