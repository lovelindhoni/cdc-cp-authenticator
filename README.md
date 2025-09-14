## Auth Routes

### GET

- **/** -> Greetings

### POST

- **/auth/leetcode**
- **/auth/codechef**
- **/auth/codeforces**

All the above takes payload in form of

```json
{
  "username": "lovelindhoni",
  "code": "late-night-lassi-sipper"
}
```

They will return

```json
{
  "status": "success" | "error",
  "message": "Authentication successful" | "Authentication failed" | "Internal server error"
}
```

> [!NOTE]
> NOTE: pass `--features local` to cargo when developing locally
