#[doc = "Register `HWCFGR6` reader"]
pub struct R(crate::R<HWCFGR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR6` writer"]
pub struct W(crate::W<HWCFGR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR6_SPEC>;
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
impl From<crate::W<HWCFGR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMAP20` reader - Input channel mapping"]
pub type CHMAP20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP20` writer - Input channel mapping"]
pub type CHMAP20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR6_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP21` reader - Input channel mapping"]
pub type CHMAP21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP21` writer - Input channel mapping"]
pub type CHMAP21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR6_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP22` reader - Input channel mapping"]
pub type CHMAP22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP22` writer - Input channel mapping"]
pub type CHMAP22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR6_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP23` reader - Input channel mapping"]
pub type CHMAP23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP23` writer - Input channel mapping"]
pub type CHMAP23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR6_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap20(&self) -> CHMAP20_R {
        CHMAP20_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap21(&self) -> CHMAP21_R {
        CHMAP21_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap22(&self) -> CHMAP22_R {
        CHMAP22_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap23(&self) -> CHMAP23_R {
        CHMAP23_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap20(&mut self) -> CHMAP20_W<0> {
        CHMAP20_W::new(self)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap21(&mut self) -> CHMAP21_W<8> {
        CHMAP21_W::new(self)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap22(&mut self) -> CHMAP22_W<16> {
        CHMAP22_W::new(self)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap23(&mut self) -> CHMAP23_W<24> {
        CHMAP23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr6](index.html) module"]
pub struct HWCFGR6_SPEC;
impl crate::RegisterSpec for HWCFGR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr6::R](R) reader structure"]
impl crate::Readable for HWCFGR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr6::W](W) writer structure"]
impl crate::Writable for HWCFGR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR6 to value 0x1f1f_1f1f"]
impl crate::Resettable for HWCFGR6_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f_1f1f;
}
