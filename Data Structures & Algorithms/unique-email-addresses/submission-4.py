class Solution:
    def numUniqueEmails(self, emails: List[str]) -> int:
        res = []
        for email in emails:
            local, domain = email.split('@')
            local = local.replace('.', '')
            if '+' in local:
                local = local.split('+')[0]
            if local + '@' + domain not in res:
                res.append(local + '@' + domain)
        return len(res)
