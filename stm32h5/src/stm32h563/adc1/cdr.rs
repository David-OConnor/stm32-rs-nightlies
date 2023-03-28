#[doc = "Register `CDR` reader"]
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA_MST` reader - Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN)) In MDMA = 0b11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
pub type RDATA_MST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RDATA_SLV` reader - Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN))"]
pub type RDATA_SLV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN)) In MDMA = 0b11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN))"]
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADC_CDR_mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](index.html) module"]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdr::R](R) reader structure"]
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDR to value 0x8000_0000"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
