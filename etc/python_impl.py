from dataclasses import dataclass
from typing import NamedTuple
import enum
from collections import defaultdict


@dataclass
class Node:
    x_i: int
    transitions: dict[str|None, 'None']
    is_accepting: bool

    def __str__(self):
        return f'(x_i={self.x_i})'


def gen_following(src: str, xi: int, si_dacc: list[tuple[int, int]], dmax: int, lookup: dict[tuple, Node]):
    trns = defaultdict(list)

    accepting = False
    for si, dacc in si_dacc:
        # genarate match
        if si >= len(src):
            # account for potential insertions at the end
            if dacc < dmax:
                trns[None].append((si, dacc + 1))
            accepting = True
            continue
        match_char = src[si]
        trns[match_char].append((si + 1, dacc))

        # generate lookaheads
        for ahead in range(1, dmax - dacc + 1):
            if si + ahead >= len(src):
                accepting = True
                continue
            cmp_char = src[si + ahead]
            if cmp_char != match_char:
                trns[cmp_char].append((si + ahead + 1, dacc + ahead))
                # matching this symbol could also represent an insert or sub
                # if the cmp_char doesnt equal the match_char
                trns[cmp_char].append((si, dacc + 1))
                trns[cmp_char].append((si + 1, dacc + 1))

        if dacc < dmax:
            # generate insertions
            trns[None].append((si, dacc + 1))
            # generate subs
            if si + 1 >= len(src):
                accepting = True
            trns[None].append((si + 1, dacc + 1))
            
    trns = dict(trns)

    key = (xi, accepting, tuple((k, tuple(v)) for k,v in trns.items()))

    if key in lookup:
        return lookup[key]
    else:
        node = Node(
            x_i=xi,
            transitions={c: gen_following(src, xi + 1, t, dmax, lookup)
                        for c, t in trns.items()},
            is_accepting=accepting)
        lookup[key] = node
        return node

def check(node: Node, input: str):
    for x in input:
        if x in node.transitions:
            node = node.transitions[x]
        elif None in node.transitions:
            node = node.transitions[None]
        else:
            return False
    return node.is_accepting

def make_lda(src, d):
    return gen_following(src, 0, [(0, 0)], d, {})

def lev_dist(a: str, b: str) -> int:
    small, large = (a, b) if len(a) < len(b) else (b, a)
    if len(small) == 0:
        return len(large)

    mtx = [[0] * (len(small) + 1) for _ in range(len(large) + 1)]

    for i in range(len(large) + 1):
        mtx[i][0] = i
    
    for i in range(len(small) + 1):
        mtx[0][i] = i

    for li in range(1, len(large) + 1):
        for si in range(1, len(small) + 1):
            mtx[li][si] = min(
                mtx[li - 1][si - 1] if large[li-1] == small[si-1] else (mtx[li - 1][si - 1] + 1),
                mtx[li][si - 1] + 1,
                mtx[li - 1][si] + 1
            )

    return mtx[-1][-1]

import random
def _test(runs_each=500):

    def _impl_gen_test_case(base: list[str], d: int, prev_edits: dict[int, str]) -> list[str]:
        if d == 0:
            return base
        etype = random.choice(['i', 'd', 's'])
        edited = False
        attempts = 10
        while not edited:
            attempts -= 1
            if attempts == 0:
                raise Exception()
            if etype == 'i':
                # cannot insert before or after deletion has previously occurred
                idx_choices = [i for i in list(range(len(base) + 1)) if
                            (prev_edits.get(i-1) != 'd') and
                            (prev_edits.get(i) != 'd') and
                            (prev_edits.get(i+1) != 'd')]
                if idx_choices:
                    idx = random.choice(idx_choices)
                    base.insert(idx, 'I')
                    prev_edits = {k if k < idx else k + 1: e for k,e in prev_edits.items()}
                    prev_edits[idx] = 'i'
                    edited = True
            elif etype == 'd':
                # cannot delete near where an insert or sub occurred, or on top of another delete
                idx_choices = [i for i in list(range(len(base))) if
                                (prev_edits.get(i - 1) not in ('i', 's')) and
                                (prev_edits.get(i) not in ('i', 's', 'd')) and
                                (prev_edits.get(i + 1) not in ('i', 's'))]
                if idx_choices:
                    idx = random.choice(idx_choices)
                    base[idx] = ''
                    prev_edits[idx] = 'd'
                    edited = True
            else:
                idx_choices = [i for  i in list(range(len(base)))
                                if (prev_edits.get(i - 1) != 'd') and
                                (not prev_edits.get(i)) and
                                (prev_edits.get(i + 1) != 'd')]
                if idx_choices:
                    idx = random.choice(idx_choices)
                    base[idx] = 'S'
                    prev_edits[idx] = 's'
                    edited = True

        return _impl_gen_test_case(base, d-1, prev_edits)


    def make_test_case(src: str, d: int) -> str:
        while True:
            try:
                tstr = ''.join(_impl_gen_test_case(list(src), d, {}))
                if lev_dist(src, tstr) == d:
                    return tstr
                else:
                    continue
            except:
                pass
    

    test_strings = [
        '',
        'aaaaaaaaaa',
        'bbbbaaaaaaa'
        'babababab'
        'abc',
        'aaabbbccc',
        'abbccc',
        'quququq',
        'asdfgaerr'
        'session',
        'lmao'
        'lol',
        'abcabcabc'
        'aaaabbbb',
        '2aj90v',
        'd4gaw',
        'dg9xx',
        'zck6om9kl',
        'nk3wadg',
        '7txelyfa5v2',
        'v6a8',
        '5',
        '9nic10',
        '8y',
        'c4ugsnjor2',
        'sao9w4v79',
        'o64hc79huh',
        'k2cy053nf',
        'l7h',
        'eytcy',
        'qk',
        'x3tr2lhyfnp',
        'n39h8tcqee',
        'xwm',
        '993xn68um',
        'fukvwehhw',
        'm6ca',
        'vbbwwszxr2',
        'sgeey',
        '4eqd',
        '26tw9qfm'
    ]
    test_strings = [s.lower() for s in test_strings]
        
    ds = [0, 1, 2, 3, 4]

    num_tests = 0

    def _run_test(lda, src, input, max_d, num_muts):
        nonlocal num_tests
        result = check(lda, input)
        accept = num_muts <= max_d
        if result != accept:
            raise AssertionError(f'FAIL: src={src}, input={input}, max_d={max_d}, num_muts={num_muts}')
        num_tests += 1
        if num_tests > 0 and num_tests % 100 == 0:
            print(f'ran {num_tests} tests')

    for src in test_strings:
        for d in ds:
            lda = make_lda(src, d)
            _run_test(lda, src, src, d, 0)
            for num_muts in range(1, d+2):
                for _ in range(runs_each):
                    _run_test(lda, src, make_test_case(src, num_muts), d, num_muts)


def _test_specific(src: str, d: int, input_str: str):
    lda = make_lda(src, d)
    
    def _check(node: Node, input: str):
        for i, x in enumerate(input):
            print(f'x{i} == {x}')
            print(f'trns = {list(node.transitions.keys())}')
            if x in node.transitions:
                node = node.transitions[x]
            elif None in node.transitions:
                node = node.transitions[None]
            else:
                return False
        return node.is_accepting
    
    result = _check(lda, input_str)
    print(f'result = {result}')


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['test', 'test-specific', 'ld'])
    parser.add_argument('--src')
    parser.add_argument('--input')
    parser.add_argument('--max-d', type=int)
    args = parser.parse_args()

    if args.action == 'test':
        _test()
    elif args.action == 'ld':
        print(lev_dist(args.src, args.input))
    else:
        _test_specific(args.src, args.max_d, args.input)