#[doc = "Register `HSEM_RLR0` reader"]
pub struct R(crate::R<HSEM_RLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_RLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_RLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_RLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCID` reader - PROCID"]
pub type PROCID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COREID` reader - COREID"]
pub type COREID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - PROCID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr0](index.html) module"]
pub struct HSEM_RLR0_SPEC;
impl crate::RegisterSpec for HSEM_RLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_rlr0::R](R) reader structure"]
impl crate::Readable for HSEM_RLR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSEM_RLR0 to value 0"]
impl crate::Resettable for HSEM_RLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}