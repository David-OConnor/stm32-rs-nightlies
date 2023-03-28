#[doc = "Register `HWCFGR5` reader"]
pub struct R(crate::R<HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR5` writer"]
pub struct W(crate::W<HWCFGR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR5_SPEC>;
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
impl From<crate::W<HWCFGR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMAP19` reader - Input channel mapping"]
pub type CHMAP19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP19` writer - Input channel mapping"]
pub type CHMAP19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP18` reader - Input channel mapping"]
pub type CHMAP18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP18` writer - Input channel mapping"]
pub type CHMAP18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP17` reader - Input channel mapping"]
pub type CHMAP17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP17` writer - Input channel mapping"]
pub type CHMAP17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP16` reader - Input channel mapping"]
pub type CHMAP16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP16` writer - Input channel mapping"]
pub type CHMAP16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR5_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap19(&self) -> CHMAP19_R {
        CHMAP19_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap18(&self) -> CHMAP18_R {
        CHMAP18_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap17(&self) -> CHMAP17_R {
        CHMAP17_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap16(&self) -> CHMAP16_R {
        CHMAP16_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap19(&mut self) -> CHMAP19_W<0> {
        CHMAP19_W::new(self)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap18(&mut self) -> CHMAP18_W<8> {
        CHMAP18_W::new(self)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap17(&mut self) -> CHMAP17_W<16> {
        CHMAP17_W::new(self)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap16(&mut self) -> CHMAP16_W<24> {
        CHMAP16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr5](index.html) module"]
pub struct HWCFGR5_SPEC;
impl crate::RegisterSpec for HWCFGR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr5::R](R) reader structure"]
impl crate::Readable for HWCFGR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr5::W](W) writer structure"]
impl crate::Writable for HWCFGR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR5 to value 0x1f08_0807"]
impl crate::Resettable for HWCFGR5_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f08_0807;
}
