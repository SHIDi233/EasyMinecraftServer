<style>
    .d{
        margin-top: 30px;
    }
</style>

<template>
    <el-card style="width: 100%">
        111
    </el-card>
</template>

<script lang="ts" setup>
    import { ref } from 'vue';
    import { reactive } from 'vue'
    import axios from 'axios'
    axios.defaults.baseURL = '/api' 
    import { ElNotification } from 'element-plus'
    import { onMounted } from 'vue';
    import type { FormInstance, FormRules } from 'element-plus'

    const dialogFormVisible=ref(false);

    const ruleFormRef = ref<FormInstance>()

    interface RuleForm {
        name:string
        phoneNumber:string
        address:string
        description:string
    };
    const form = reactive<RuleForm>({
        name: '',
        phoneNumber: '',
        address:'',
        description:'',
    });
    const tableData = ref([
        {
            date: '',
            name: '',
            address: '',
        }
    ]);
    const rules = reactive<FormRules<RuleForm>>({
        name:[
            { required: true, message: 'Please input name', trigger: 'blur'},
            { min: 3, max: 5, message: 'Length should be 3 to 5', trigger: 'blur' },
        ],
        phoneNumber:[
            { required: true, message: 'Please input phoneNumber', trigger: 'blur'},
        ],
        address:[
            { required: true, message: 'Please input address', trigger: 'blur'},
        ],
        description:[
            { required: true, message: 'Please input description', trigger: 'blur'},
        ],
    });

    const loading=ref(true);
    const loading_2=ref(false);

    const refresh=()=>{
        loading.value=true;
        axios.get('/warehouses')
        .then(function (res) {
            if(res.data.message=='ok'){
                tableData.value = res.data.data;
                loading.value=false;
            }
            else{
                ElNotification({
                    title: '错误',
                    message: res.data.data,
                    type: 'error',
                })
            }
        })
        .catch(function (error) {
            ElNotification({
                title: '网络错误',
                message: error,
                type: 'error',
            })
        });
    };

    onMounted(() => {
        refresh();
    });

    const commit=(formEl: FormInstance | undefined)=>{
        if (!formEl) {alert(2);return;}
        formEl.validate((valid, fields) => {
            if (valid) {
                loading_2.value=true;
                axios.post('/warehouses', 
                { "name":form.name,"description": form.description,
                    "address":form.address,"phoneNumber":form.phoneNumber})
                .then(function (res) {
                    if(res.data.message=='ok'){
                        ElNotification({
                            title: 'Success',
                            message: '创建成功',
                            type: 'success',
                        });
                        dialogFormVisible.value=false;
                        loading_2.value=false;
                        refresh();
                    }
                    else{
                        ElNotification({
                            title: '创建错误',
                            message: res.data.data,
                            type: 'error',
                        })
                        loading_2.value=false;
                    }
                })
                .catch(function (error) {
                    ElNotification({
                        title: '网络错误',
                        message: error,
                        type: 'error',
                    })
                    loading_2.value=false;
                });
            } else {
                return;
            }
        })
    }
</script>