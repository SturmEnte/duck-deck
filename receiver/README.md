## Config

### Example Config:

```json
[
	{
		"id": "1",
		"action": {
			"type": "key-press",
			"key": "^",
			"ctrl": false,
			"alt": false,
			"shift": false
		}
	}
]
```

This config means that when the first (top-left) button is pressed, the ^ key will be pressed. It's important to always set all values of the action type, other wise the receiver will crash on startup.

### All possible action types and their values:

Key Press:
If ctrl, alt and/or shift are set to true, the corresponding keys are pressed before the key is pressed and released after the key was pressed.
Structure of this type:

```json
{
	"type": "key-press",
	"key": "[The key that shall be pressed]",
	"ctrl": false,
	"alt": false,
	"shift": false
}
```
