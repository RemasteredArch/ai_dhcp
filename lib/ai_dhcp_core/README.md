# `ai_dhcp_core`

`ai_dhcp_core` is the actual DHCP implementation of `ai_dhcp`.

## Architecture

`ai_dhcp_core` is not yet complete, but the rough architecture is as follows:

- `ai_dhcp_core` parses the body of a UDP datagram,
  deciding whether it is a DHCP discovery message, an extension of a current session, or not DHCP at all.
- Clients requesting a DHCP lease have their information passed to the consumer's implementation of `LeaseProvider`.
  The consumer is responsible for ensuring that their implementation of `LeaseProvider` does not provide duplicate assignments.
  For `ai_dhcp`, this provider is responsible for prompting an LLM,
  and intentionally does _not_ ensure that it does not provide duplicate assignments
  with the intention that it causes chaos.

## License

`ai_dhcp` is licensed under the Mozilla Public License,
version 2.0 or (as the license stipulates) any later version.
A copy of the license should be distributed with `ai_dhcp`,
located at [`LICENSE`](../../LICENSE),
or you can obtain one at <https://mozilla.org/MPL/2.0/>.
