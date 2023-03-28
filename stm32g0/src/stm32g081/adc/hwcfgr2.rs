#[doc = "Register `HWCFGR2` reader"]
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR2` writer"]
pub struct W(crate::W<HWCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR2_SPEC>;
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
impl From<crate::W<HWCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMAP7` reader - Input channel mapping"]
pub type CHMAP7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP7` writer - Input channel mapping"]
pub type CHMAP7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP6` reader - Input channel mapping"]
pub type CHMAP6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP6` writer - Input channel mapping"]
pub type CHMAP6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP5` reader - Input channel mapping"]
pub type CHMAP5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP5` writer - Input channel mapping"]
pub type CHMAP5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP4` reader - Input channel mapping"]
pub type CHMAP4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP4` writer - Input channel mapping"]
pub type CHMAP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap7(&self) -> CHMAP7_R {
        CHMAP7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap6(&self) -> CHMAP6_R {
        CHMAP6_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap5(&self) -> CHMAP5_R {
        CHMAP5_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap4(&self) -> CHMAP4_R {
        CHMAP4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap7(&mut self) -> CHMAP7_W<0> {
        CHMAP7_W::new(self)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap6(&mut self) -> CHMAP6_W<8> {
        CHMAP6_W::new(self)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap5(&mut self) -> CHMAP5_W<16> {
        CHMAP5_W::new(self)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap4(&mut self) -> CHMAP4_W<24> {
        CHMAP4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](index.html) module"]
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr2::R](R) reader structure"]
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr2::W](W) writer structure"]
impl crate::Writable for HWCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR2 to value 0x0505_0404"]
impl crate::Resettable for HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0505_0404;
}
