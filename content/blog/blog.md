```base
filters:
  and:
    - file.folder == "server/content/blog"
    - file.name != "blog"
views:
  - type: table
    name: Table
    order:
      - file.name
      - tags
      - created
      - author

```
