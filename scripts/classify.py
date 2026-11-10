import sys,re,json; import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import load
it=load()
TOPICS=[
 ('Loops (for/while/loop, break/continue, fixed point, loop contexts)', r'\bloops?\b|for loop|`for`|\bbreak\b|\bcontinue\b|fixed[- ]?point|fixpoint|match(ing)? (the )?contexts|join(ing)? .*context'),
 ('Iterators & ranges (Iterator, IntoIterator, Range, adapters)', r'iterat|IntoIter|\brange|RangeFrom|RangeTo|\bStep\b|\bzip\b|enumerate|\bfold\b|\bmap\b|filter|collect|\bsum\b|take_?while|chain|\brev\b|next_back|try_for_each|\.iter\(|Iter::'),
 ('Closures / Fn traits / function pointers', r'closure|\bFn(Mut|Once)?\b|impl Fn|function pointer|call_once|upvar'),
 ('dyn Trait / trait objects / vtables', r'\bdyn\b|vtable|trait object'),
 ('Traits: associated types, GATs, HRTB, default methods, specialization', r'associated type|\bGATs?\b|higher.ranked|HRTB|for<\'a>|default method|supertrait|specialis|specializ|trait bound|trait impl|marker trait|RPITIT|impl trait'),
 ('Unsafe / raw pointers / unions / MaybeUninit / FFI / extern / asm', r'unsafe|raw.pointer|\*const|\*mut|\bunion|MaybeUninit|\bFFI\b|extern|inline asm|NonNull|heap model|no_core'),
 ('Interior mutability / Rc / Cell / RefCell / Mutex / atomics / Drop', r'\bRc\b|RefCell|\bCell\b|Mutex|atomic|\bArc\b|\bDrop\b|destructor|drop glue|drops'),
 ('Async / coroutines / threads', r'\basync\b|coroutine|thread.local|thread'),
 ('Borrows: nested borrows, lifetimes, \'static, regions, abstractions', r'nested (mutable |shared )?borrow|nested reference|lifetime|\'static|region|&mut &mut|borrow|abstraction|loan'),
 ('Recursion / mutual recursion / termination / divergence / partial_fixpoint', r'recurs|mutual|terminat|diverg|partial_fixpoint|strict positivity|nonpos'),
 ('Const generics / consts / statics / globals', r'const generic|\bconst\b|\bstatic\b|global'),
 ('Strings / str / char / C strings', r'\bString\b|\bstr\b|string|\bchar\b|CStr|C-string|toStr'),
 ('Collections: Vec / slice / array / Box', r'\bVec\b|vec!|slice|\barray|\bBox\b|from_fn'),
 ('Casts / arithmetic / overflow / floats / scalars', r'\bcasts?\b|\bas\b `?|overflow|wrapping|checked|saturat|float|\bshift|scalar|\brem\b|division|discriminant'),
 ('Std library models / opaque / external definitions', r'\bstd\b|\bcore\b|opaque|axiom|external|foreign|model'),
 ('Crashes / panics / internal errors / unsupported (generic)', r'panic|crash|internal error|uncaught|unreachable|unimplemented|not supported|unsupported|\bICE\b|assert|failure|CFailure|Not_found|stack overflow|OOM'),
]
BODY={'Loops':r'\bloop \{|\bwhile \w|for \w+ in |\bbreak\b','Iterators':r'\.iter\(|\.into_iter\(|\.iter_mut\(|Iterator|\.zip\(|\.enumerate\(|\.fold\(|\.collect\(|\.sum\(|for \w+ in \d+\.\.','Closures':r'closure','dyn':r'\bdyn \w','Unsafe':r'\bunsafe \{|raw pointer','Interior':r'RefCell|\bRc<|\bCell<|Mutex|Atomic','Async':r'\basync fn'}
def note(x):
    b=(x.get('body') or '').replace('\r','')
    for line in b.split('\n'):
        l=line.strip().strip('#').strip()
        if len(l)>15 and not l.startswith(('```','<!--','|','- [','Co-Authored','<img','![')):
            return l[:160]
    return ''
def row(x):
    merged=''
    if x['is_pr']:
        m=x['pull_request'].get('merged_at')
        st='open' if x['state']=='open' else ('merged' if m else 'closed (unmerged)')
    else: st=x['state']+('' if x['state']=='open' else (' ('+x.get('state_reason','') +')' if x.get('state_reason') else ''))
    url=x['html_url']; t=x['title'].replace('|','\\|')
    n=note(x).replace('|','\\|')
    return f"| [{x['repo']}#{x['number']}]({url}) | {t} | {st} | {x['created_at'][:10]} | {(x['closed_at'] or '')[:10]} | {n} |"
out=[]; used=set(); counts={}
sel=[x for k,x in it.items() if not (x['state']=='closed' and x['closed_at']<'2025-11-10')]
for name,pat in TOPICS:
    p=re.compile(pat,re.I)
    rows=[]
    for x in sorted(sel,key=lambda x:(x['repo'],-x['number'])):
        title_hit=p.search(x['title'])
        bp=BODY.get(name.split(' ')[0])
        body_hit=(not x['is_pr']) and bp and re.search(bp,(x.get('body') or '')[:4000])
        if title_hit or body_hit:
            rows.append(row(x)+('' if title_hit else ' (body match)'))
    counts[name]=len(rows)
    out.append(f"\n### {name} ({len(rows)} items)\n\n| # | Title | State | Created | Closed | Note (first line of body) |\n|---|---|---|---|---|---|\n"+'\n'.join(rows))
open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','notes','topic_tables.md'),'w').write('\n'.join(out))
for k,v in counts.items(): print(v,k)
