class Solution:
    def numUniqueEmails(self, emails: List[str]) -> int:
        mp = set()
        for email in emails:
            local, domain = email.split('@')
            local = local.split('+')[0]
            local = local.replace('.', '')
            mp.add((local, domain))
        return len(mp)
