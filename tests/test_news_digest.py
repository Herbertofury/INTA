import copy
import json
import unittest
from test_hub import ROOT, build

class NewsDigestTest(unittest.TestCase):
    def setUp(self):
        self.news=json.loads((ROOT/'data/news-sources.json').read_text())
        self.snapshot=json.loads((ROOT/'data/news-digest.json').read_text())

    def test_contract_and_separate_native_artifact(self):
        build.validate_news_digest(self.snapshot,self.news)
        outputs=build.build_outputs()
        self.assertEqual(json.loads(outputs['site/news-digest.json']),self.snapshot)
        self.assertEqual(json.loads(outputs['site/catalog.json'])['newsDigest'],self.snapshot)

    def test_source_freshness_is_optional_and_qualified(self):
        obj=copy.deepcopy(self.snapshot)
        by_id={row["id"]:row for row in obj["sources"]}
        by_id["windows-health"]["origin"]="editorial"
        by_id["github-status"].update(origin="feed",lastSuccessfulCheckAt=obj["generatedAt"])
        by_id["bleeping"].update(origin="feed",lastSuccessfulCheckAt="2026-11-26T19:00:00Z")
        build.validate_news_digest(obj,self.news)
        bad=[("windows-health","lastSuccessfulCheckAt",obj["generatedAt"]),
             ("github-status","origin","editorial"),
             ("github-status","lastSuccessfulCheckAt","2026-11-25T19:00:00Z"),
             ("bleeping","lastSuccessfulCheckAt","2026-11-29T05:00:00Z"),
             ("bleeping","lastSuccessfulCheckAt","2026-11-26T19:00:00+00:00"),
             ("bleeping","lastSuccessfulCheckAt",None)]
        for ident,key,value in bad:
            scoped=copy.deepcopy(obj)
            next(row for row in scoped["sources"] if row["id"]==ident)[key]=value
            with self.subTest(ident=ident,key=key,value=value),self.assertRaises(ValueError):
                build.validate_news_digest(scoped,self.news)

    def test_invalid_media_link_and_cross_source(self):
        bad_pairs=[('url','http://publisher.example/item'),('url','https://user@publisher.example/item'),
                   ('sourceId','not-a-source'),('date','2099-03-01'),
                   ('media',[{'kind':'embed','url':'https://video.invalid/iframe'}]),
                   ('media',[{'kind':'video','url':'javascript:alert(1)'}]),
                   ('media',[{'kind':'image','url':'https://unknown.example/private.png'}]),
                   ('url','https://githubstatus.com.example.com/update'), ('id','forged-identity')]
        for key,value in bad_pairs:
            obj=copy.deepcopy(self.snapshot); obj['items'][0][key]=value
            with self.subTest(key=key,value=value),self.assertRaises(ValueError):
                build.validate_news_digest(obj,self.news)
