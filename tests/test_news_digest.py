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
