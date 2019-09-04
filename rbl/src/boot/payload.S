.section ".payload", "a", @progbits
.align 17
.global _payload_start
_payload_start:
    .incbin "target/rcore"
.global _payload_end
_payload_end:
    .byte 0