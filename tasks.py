from invoke import task


@task
def install(c):
    "install"
    c.run('cargo install --path .')


@task(default=True)
def usage(c):
    """Usage"""

    c.run('invoke -l')
