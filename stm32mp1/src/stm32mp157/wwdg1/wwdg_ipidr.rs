#[doc = "Register `WWDG_IPIDR` reader"]
pub struct R(crate::R<WWDG_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "WWDG ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_ipidr](index.html) module"]
pub struct WWDG_IPIDR_SPEC;
impl crate::RegisterSpec for WWDG_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_ipidr::R](R) reader structure"]
impl crate::Readable for WWDG_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WWDG_IPIDR to value 0x0012_0051"]
impl crate::Resettable for WWDG_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0012_0051;
}
