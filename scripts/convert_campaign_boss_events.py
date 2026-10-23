"""Convert existing nightly campaign events; dry-run unless --apply is given."""
import argparse
import copy
import gzip
import json
import shutil
import subprocess
import time
from pathlib import Path

MAPPING = {'Regular': 'Boss', 'Dead Lands Regular': 'Dead Lands Boss',
           'Norselands Regular': 'Norselands Boss', 'Plague_Regular': 'Plague_BloodMoon',
           'Trade Routes Regular': 'Trade Routes Boss', 'SkullIsland': 'SkullIsland Boss',
           'Anniversary_Regular_Wave': 'Anniversary_Boss_Wave', 'DireIslandRegular': 'DireIslandBloodMoon'}

def convert(doc):
    result = copy.deepcopy(doc)
    changed = []
    for ci, campaign in enumerate(result.get('campaigns', [])):
        for island in campaign.get('_islands', []):
            for obj in island.get('objects', []):
                for comp in obj.get('componentData2', []):
                    if comp.get('type') != 'DirectorData':
                        continue
                    raw = comp.get('data')
                    data = json.loads(raw) if isinstance(raw, str) else raw
                    if not isinstance(data, dict):
                        raise ValueError('Invalid DirectorData')
                    count = 0
                    for event in data.get('events', []):
                        old = event.get('waveName')
                        if event.get('type') in (2, 4) and old in MAPPING:
                            event['waveName'] = MAPPING[old]
                            count += 1
                    if count:
                        comp['data'] = json.dumps(data, separators=(',', ':')) if isinstance(raw,str) else data
                        changed.append((ci+1, island.get('land'), count))
    return result, changed

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('save', type=Path)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    original_bytes = args.save.read_bytes()
    original = json.loads(gzip.decompress(original_bytes))
    result, changes = convert(original)
    print('Campaign/island/events:', changes)
    assert result.get('challenges') == original.get('challenges')
    assert convert(result)[1] == []
    if args.apply and changes:
        if subprocess.run(['pgrep','-f','Contents/MacOS/KingdomTwoCrowns'],capture_output=True).returncode == 0:
            raise RuntimeError('Close the game first')
        stamp = time.time_ns()
        backup = args.save.with_name(args.save.name+f'.before-boss-events-{stamp}.bak')
        temp = args.save.with_name(args.save.name+f'.boss-events-{stamp}.tmp')
        shutil.copy2(args.save, backup)
        try:
            temp.write_bytes(gzip.compress(json.dumps(result,separators=(',',':')).encode()))
            assert json.loads(gzip.decompress(temp.read_bytes())) == result
            assert args.save.read_bytes() == original_bytes, 'Save changed concurrently'
            shutil.copymode(args.save,temp)
            temp.replace(args.save)
        finally:
            temp.unlink(missing_ok=True)
        print('Written and verified. Backup:', backup)
