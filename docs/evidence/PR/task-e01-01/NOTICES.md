# Notices do spike E01-01

Não é THIRD_PARTY_NOTICES final nem autorização de distribuição do produto. A licença final do código Vynamp permanece decisão pendente do mantenedor.

Symphonia0.6.1 e seus componentes não modificados: MPL-2.0, The Project Symphonia Developers, origem https://github.com/pdeljanov/Symphonia. [Inventário de dependências](dependencies.json) gerado do cargo metadata/resolve normal; contém nome, versão, declaração de licença e origem de cada pacote transitivo, sem afirmar auditoria de todos os termos, patentes ou CVEs. Licenças completas permanecem nos pacotes de origem; shipping exige incorporar notices/textos aplicáveis e cumprir a MPL quando pertinente. Features exatas e checagem numérica documentadas no ADR-002.

Ferramenta externa: FFmpeg9.0.2 Gyan essentials, GPLv3 segundo fornecedor, build --enable-gpl --enable-version3 com libmp3lame. Usado apenas para gerar/decodificar sinais sintéticos de teste; executável/ZIP não entram em Git, linking ou runtime Vynamp. Registro de versão/configuração/hash no manifesto, ferramenta na scratch ignored. Não inferir permissão de shipping ou herança de capacidade de codec por essa ferramenta. Referências: https://ffmpeg.org/download.html e https://www.gyan.dev/ffmpeg/builds/.

Corpus: sinais gerados para o repositório por generate.ps1, sem gravações comerciais/dados pessoais/Winamp. Manifesto registra origem, permissão de uso como dados de teste, transformação e hashes. Isso não seleciona uma licença final para fontes/assets do produto. SHA e resultados são dos bytes incluídos, não de URLs mutáveis.
