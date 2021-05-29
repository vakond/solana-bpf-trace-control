# Controlling BPF Trace

During execution the runtime BPF interpreter can be configured to log a trace message for each BPF instruction executed. This can be very helpful for things like pin-pointing the runtime context leading up to a memory access violation.

The trace logs together with the ELF dump can provide a lot of insight.

To turn on BPF interpreter trace messages in a local cluster configure the `solana_rbpf` level in `RUST_LOG` to trace. For example:
```
export RUST_LOG=solana_bpf_loader_program=trace
```

There is additional level of control of this capability. To enable it the Solana local cluster must be built with certain feature `with-bpf-trace-control`:
```
cargo build --release --features with-bpf-trace-control
```

To turn it on create another environment variable `SOLANA_BPF_TRACE_CONTROL`, which should contain a TCP port. For example:
```
export SOLANA_BPF_TRACE_CONTROL="8888"
```

The control config contains several settings to change behaviour of BPF tracing without restart of a local cluster. Here is the default config how it can be viewed with the `solana-bpf-trace-control` utility (see later):
```
enable = true
filter = 
output = /tmp/trace
multiple = true
```
It enables tracing without filtering, and redirects output of each execution into a different file.

Here is a brief description of the settings.

### Enable or disable BPF tracing altogether
For example:
```
enable = true
```

### Filter BPF tracing of a certain program
For example:
```
filter = evm_loader:3CMCRJieHS3sWWeovyFyH4iRyX4rHf3u2zbC5RCFrRex
```
The first part of `filter` setting is arbitrary string to be included to the trace file name. The second part is a program id of interest. Empty `filter` setting passes any program. _Caution_: irrelevant information in the `filter` blocks any tracing.

### Redirect BPF tracing into a file
For example:
```
trace-file = /tmp/trace
```
Empty `trace-file` setting means the tracing will be dumped to the standard output.

### Separate BPF traces of different runs
For example:
```
multuple = true
```
Each new BPF trace will be written into a separate file. Names of these files will contain timestamps. Otherwise the `multiple` setting causes appending new traces all to a single file (if `trace-file` presents).

### The control tool: solana-bpf-trace-control
There exists a dedicated tool to view (a command without argument) or change (a command with argument) the settings. Here are some usage examples:
```
solana-bpf-trace-control show
```
```
solana-bpf-trace-control enable
```
```
solana-bpf-trace-control enable false
```
```
solana-bpf-trace-control filter 'evm_loader:3CMCRJieHS3sWWeovyFyH4iRyX4rHf3u2zbC5RCFrRex'
```
```
solana-bpf-trace-control trace-file ''
```
```
solana-bpf-trace-control multiple false
```
_Note_: to clear a string setting just pass empty quoted string as the argument.

### Performance issues

Writing a large trace file could be a substantially heavy task for the system. It's better to redirect output to a RAM disk if possible.

The controlling service of the local custer launches an OS thread and binds a socket. To minimize possible performance impact the service will actually start only after the first BPF execution. The default settings will be taken into account for this first execution. _Advice_: first execute a small program like `memo` or `token` to make sure the controlling service is up and running.
