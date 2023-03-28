#[doc = "Register `GICV_AIAR` reader"]
pub struct R(crate::R<GICV_AIAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_AIAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_AIAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_AIAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERRUPT_ID` reader - INTERRUPT_ID"]
pub type INTERRUPT_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CPUID` reader - CPUID"]
pub type CPUID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:9 - INTERRUPT_ID"]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "GICV VM aliased interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_aiar](index.html) module"]
pub struct GICV_AIAR_SPEC;
impl crate::RegisterSpec for GICV_AIAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_aiar::R](R) reader structure"]
impl crate::Readable for GICV_AIAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICV_AIAR to value 0x03ff"]
impl crate::Resettable for GICV_AIAR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
