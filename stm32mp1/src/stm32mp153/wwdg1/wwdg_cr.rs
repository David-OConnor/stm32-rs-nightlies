#[doc = "Register `WWDG_CR` reader"]
pub struct R(crate::R<WWDG_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDG_CR` writer"]
pub struct W(crate::W<WWDG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WWDG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WWDG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T` reader - T"]
pub type T_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T` writer - T"]
pub type T_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WWDG_CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `WDGA` reader - WDGA"]
pub type WDGA_R = crate::BitReader<bool>;
#[doc = "Field `WDGA` writer - WDGA"]
pub type WDGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WWDG_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - T"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - WDGA"]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - T"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<0> {
        T_W::new(self)
    }
    #[doc = "Bit 7 - WDGA"]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<7> {
        WDGA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cr](index.html) module"]
pub struct WWDG_CR_SPEC;
impl crate::RegisterSpec for WWDG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_cr::R](R) reader structure"]
impl crate::Readable for WWDG_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdg_cr::W](W) writer structure"]
impl crate::Writable for WWDG_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WWDG_CR to value 0x7f"]
impl crate::Resettable for WWDG_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
