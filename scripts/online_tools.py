"""One source for INTA's online service directory and task-oriented launchers."""
from __future__ import annotations

import re
from datetime import date
from urllib.parse import urlparse

HUB = "https://herbertofury.github.io/INTA/"
WIKI = "https://github.com/Herbertofury/INTA/wiki/"


def validate_online(data, tools, jobs):
    def require_text(value, label):
        if not isinstance(value, str) or not value.strip():
            raise ValueError(f'{label}: nonempty text is required')

    def identity(value, label):
        if not isinstance(value, str) or not re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', value):
            raise ValueError(f'{label}: a stable slug ID is required')

    def reviewed(value, label):
        try:
            if not isinstance(value, str) or not re.fullmatch(r'\d{4}-\d{2}-\d{2}', value):
                raise ValueError()
            parsed = date.fromisoformat(value)
            if parsed > date.today():
                raise ValueError()
            return parsed
        except (TypeError, ValueError):
            raise ValueError(f'{label}: invalid or future review date') from None

    if data.get('schemaVersion') != 1:
        raise ValueError('Online tools require schemaVersion 1')
    review_date = reviewed(data.get('reviewedAt'), 'Online directory')
    for key in ('groups', 'tools', 'workflows', 'featured'):
        if not isinstance(data.get(key), list) or not data[key]:
            raise ValueError(f'Online {key} must be a nonempty list')
    catalog = {tool['id']: tool for tool in tools}
    job_ids = {job['id'] for job in jobs}
    groups = {group['id']: group for group in data['groups']}
    entries = data['tools']
    if len(groups) != len(data['groups']) or len({entry['toolId'] for entry in entries}) != len(entries):
        raise ValueError('Online tools and groups require unique identities')
    for group in data['groups']:
        identity(group.get('id'), 'Online group')
        for key in ('label', 'summary'):
            require_text(group.get(key), f"{group['id']}.{key}")
    online_ids = {entry['toolId'] for entry in entries}
    if len(set(data['featured'])) != len(data['featured']) or any(item not in online_ids for item in data['featured']):
        raise ValueError('Featured online tools must be unique and exist in the online catalog')
    if {entry['toolId'] for entry in entries} != {tool['id'] for tool in tools if tool['category'] == 'online'}:
        raise ValueError('Online metadata must cover every online catalog tool exactly once')
    for entry in entries:
        tool_id = entry['toolId']
        identity(tool_id, 'Online tool')
        if tool_id not in catalog or entry['group'] not in groups:
            raise ValueError(f'Unknown online relationship: {tool_id}')
        for field in ('account', 'sharing', 'firstAction', 'expectedResult', 'sources', 'reviewedAt'):
            if not entry.get(field):
                raise ValueError(f'{tool_id}: missing online {field}')
            if field != 'sources':
                require_text(entry[field], f'{tool_id}.{field}')
        if catalog[tool_id]['mode'] != 'web':
            raise ValueError(f'{tool_id}: online tools must be labeled web services')
        if reviewed(entry['reviewedAt'], tool_id) > review_date:
            raise ValueError(f'{tool_id}: review date is newer than the directory review')
        if not isinstance(entry['sources'], list):
            raise ValueError(f'{tool_id}: sources must be a list')
        for source in entry['sources']:
            parsed = urlparse(source.get('url', ''))
            if (parsed.scheme != 'https' or not parsed.hostname or parsed.username or parsed.password
                    or any(key in parsed.query.lower() for key in ('utm_', 'fbclid=', 'gclid='))):
                raise ValueError(f'{tool_id}: each claim needs a public HTTPS source without tracking')
            require_text(source.get('title'), f'{tool_id}.source.title')
            require_text(source.get('evidence'), f'{tool_id}.source.evidence')
    covered = {entry['group'] for entry in entries}
    if covered != set(groups):
        raise ValueError('Every online group must contain tools')
    workflow_ids = set()
    for workflow in data['workflows']:
        identity(workflow.get('id'), 'Online workflow')
        for key in ('title', 'summary'):
            require_text(workflow.get(key), f"{workflow['id']}.{key}")
        if workflow['id'] in workflow_ids or workflow['jobId'] not in job_ids:
            raise ValueError('Online workflows require unique IDs and an existing full guide')
        workflow_ids.add(workflow['id'])
        if (not isinstance(workflow.get('steps'), list) or len(workflow['steps']) != 3
                or not isinstance(workflow.get('toolIds'), list) or not workflow['toolIds']
                or len(set(workflow['toolIds'])) != len(workflow['toolIds'])
                or any(tool_id not in online_ids for tool_id in workflow['toolIds'])):
            raise ValueError(f"{workflow['id']}: use three first actions and real catalog tools")
        for step in workflow['steps']:
            require_text(step, f"{workflow['id']}.step")


def cell(value):
    return str(value).replace('|', '\\|').replace('\n', ' ')


def tool_link(identity, tools, access):
    return f"[{tools[identity]['name']}]({access[identity]['primary']['url']})"


def start_markdown(data, tools, access):
    lines = ['## 🌐 Online tools: open a service and get a result', '',
             f"**[Browse all {len(data['tools'])} online tools]({HUB}#online)** · **[Online tools Wiki]({WIKI}Online-Tools)**", '',
             '| Your job | Open the right tools | First action |', '| --- | --- | --- |']
    for workflow in data['workflows']:
        lines.append(f"| **[{workflow['title']}]({HUB}#online?task={workflow['id']})** | "
                     + ' · '.join(tool_link(identity, tools, access) for identity in workflow['toolIds'])
                     + f" | {cell(workflow['steps'][0])} |")
    lines += ['', 'Service links open their official websites. Account, access and submission visibility are shown beside the tools in the directory.', '']
    return '\n'.join(lines)


def wiki_outputs(data, catalog, access):
    tools = {tool['id']: tool for tool in catalog}
    entries = {entry['toolId']: entry for entry in data['tools']}
    lines = ['# 🌐 Online tools', '',
             f"**{len(entries)} services across {len(data['groups'])} practical groups. Source review: {data['reviewedAt']}.**", '',
             f"**[Open the searchable working directory]({HUB}#online)** · [Complete tool catalog](Tool-Directory) · [Home](Home)", '',
             'Choose the job first. Each service has a direct launch, a concrete first action, an expected result and its account and sharing behavior. The date records an editorial source review, not a claim that every service is a new release or was tested with a live incident.', '',
             start_markdown(data, tools, access),
             '## Browse by task', '',
             *[f"- **[{group['label']}](Online-{group['id']})**: {group['summary']}" for group in data['groups']], '',
             '## Work through a result', '']
    for workflow in data['workflows']:
        lines += [f"<a id=\"{workflow['id']}\"></a>", '', f"### {workflow['title']}", '', workflow['summary'], '',
                  ' · '.join(tool_link(identity, tools, access) for identity in workflow['toolIds']), '',
                  *[f'{number}. {step}' for number, step in enumerate(workflow['steps'], 1)], '',
                  f"**Continue:** [Complete job guide]({WIKI}Job-{workflow['jobId']}) · [Interactive toolkit]({HUB}#job/{workflow['jobId']})", '']
    outputs = {'wiki/Online-Tools.md': '\n'.join(lines)}
    for group in data['groups']:
        group_entries = [entry for entry in data['tools'] if entry['group'] == group['id']]
        content = [f"# {group['label']}", '', group['summary'], '',
                   f"**[Filter these tools in the app]({HUB}#online?group={group['id']})** · [All online tools](Online-Tools)", '',
                   ' | '.join(tool_link(entry['toolId'], tools, access) for entry in group_entries), '']
        for entry in group_entries:
            identity = entry['toolId']
            tool = tools[identity]
            content += [f'<a id="{identity}"></a>', '', f"## {tool_link(identity, tools, access)}", '', tool['summary'], '',
                        f"**[Open service]({access[identity]['primary']['url']})** · [Official page]({access[identity]['officialUrl']}) · [Documentation]({tool['docsUrl']})", '',
                        f"**Use for:** {tool['bestFor']}", '', f"**Start here:** {entry['firstAction']}", '',
                        f"**Read the result:** {entry['expectedResult']}", '',
                        f"**Access:** {tool['pricing']} {entry['account']}", '',
                        f"**Submission behavior:** {entry['sharing']}", '',
                        '**Official evidence:**', '',
                        *[f"- [{source['title']}]({source['url']}): {source['evidence']}" for source in entry['sources']], '',
                        f"Reviewed {entry['reviewedAt']}. [Back to all online tools](Online-Tools).", '']
        outputs[f"wiki/Online-{group['id']}.md"] = '\n'.join(content)
    return outputs
