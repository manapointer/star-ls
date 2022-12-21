import {
  ExecutableOptions,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate() {
  const command = process.env.__STAR_LSP_SERVER_DEBUG;
  if (!command) {
    throw new Error('star-ls command is not set correctly');
  }

  const options: ExecutableOptions = {
    env: {
      'RUST_BACKTRACE': '1',
    },
  };

  // TODO: Add debug support.
  const serverOptions: ServerOptions = { command, options };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'starlark' }],
  };

  client = new LanguageClient(
    'star-ls',
    'Starlark Language Server',
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
