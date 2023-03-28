#[doc = "Register `PRAR_CUR` reader"]
pub struct R(crate::R<PRAR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRAR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRAR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRAR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROT_AREA_START` reader - Bank 1 lowest PCROP protected address"]
pub type PROT_AREA_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PROT_AREA_END` reader - Bank 1 highest PCROP protected address"]
pub type PROT_AREA_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMEP` reader - Bank 1 PCROP protected erase enable option status bit"]
pub type DMEP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FLASH protection address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prar_cur](index.html) module"]
pub struct PRAR_CUR_SPEC;
impl crate::RegisterSpec for PRAR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prar_cur::R](R) reader structure"]
impl crate::Readable for PRAR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRAR_CUR to value 0"]
impl crate::Resettable for PRAR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
