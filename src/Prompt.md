## Splitr

Write a performant rust program that will split very large files of different types of formats  into manageable parts. It should have the ability to save locally (default to $DATADUMP env var for filesystem dump) and local Minio as well as the three object storage cloud provider AWS, GCP and Azure Storage accounts. Add logic for all the providers. Provide for the parameterization of the size of the split files passed as parameters in TB GB and MB as well as the very detailed documentation presented as markdown.