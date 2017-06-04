SELECT event, {&a}.t AT TIME ZONE 'UTC', {&c}.data
  FROM {&a} JOIN {&b} USING event JOIN {&c} USING event
 WHERE {&a}.token = {token}
   AND {&b}.token = {token}
   AND {&c}.token = {token}
   AND {&c}.tag = ANY ({tags})