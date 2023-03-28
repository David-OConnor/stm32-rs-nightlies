#[doc = "Register `HWCFGR3` reader"]
pub struct R(crate::R<HWCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR3` writer"]
pub struct W(crate::W<HWCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR3_SPEC>;
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
impl From<crate::W<HWCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMAP11` reader - Input channel mapping"]
pub type CHMAP11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP11` writer - Input channel mapping"]
pub type CHMAP11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP10` reader - Input channel mapping"]
pub type CHMAP10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP10` writer - Input channel mapping"]
pub type CHMAP10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP9` reader - Input channel mapping"]
pub type CHMAP9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP9` writer - Input channel mapping"]
pub type CHMAP9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHMAP8` reader - Input channel mapping"]
pub type CHMAP8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMAP8` writer - Input channel mapping"]
pub type CHMAP8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap11(&self) -> CHMAP11_R {
        CHMAP11_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap10(&self) -> CHMAP10_R {
        CHMAP10_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap9(&self) -> CHMAP9_R {
        CHMAP9_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap8(&self) -> CHMAP8_R {
        CHMAP8_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap11(&mut self) -> CHMAP11_W<0> {
        CHMAP11_W::new(self)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap10(&mut self) -> CHMAP10_W<8> {
        CHMAP10_W::new(self)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap9(&mut self) -> CHMAP9_W<16> {
        CHMAP9_W::new(self)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap8(&mut self) -> CHMAP8_W<24> {
        CHMAP8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr3](index.html) module"]
pub struct HWCFGR3_SPEC;
impl crate::RegisterSpec for HWCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr3::R](R) reader structure"]
impl crate::Readable for HWCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr3::W](W) writer structure"]
impl crate::Writable for HWCFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR3 to value 0x0706_0605"]
impl crate::Resettable for HWCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0706_0605;
}
