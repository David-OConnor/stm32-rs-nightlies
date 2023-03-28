#[doc = "Register `IOPRSTR` reader"]
pub struct R(crate::R<IOPRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPRSTR` writer"]
pub struct W(crate::W<IOPRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPRSTR_SPEC>;
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
impl From<crate::W<IOPRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOARST` reader - GPIOARST"]
pub type GPIOARST_R = crate::BitReader<bool>;
#[doc = "Field `GPIOARST` writer - GPIOARST"]
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `GPIOBRST` reader - GPIOBRST"]
pub type GPIOBRST_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBRST` writer - GPIOBRST"]
pub type GPIOBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `GPIOCRST` reader - GPIOCRST"]
pub type GPIOCRST_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCRST` writer - GPIOCRST"]
pub type GPIOCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `GPIODRST` reader - GPIODRST"]
pub type GPIODRST_R = crate::BitReader<bool>;
#[doc = "Field `GPIODRST` writer - GPIODRST"]
pub type GPIODRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `GPIOERST` reader - GPIOERST"]
pub type GPIOERST_R = crate::BitReader<bool>;
#[doc = "Field `GPIOERST` writer - GPIOERST"]
pub type GPIOERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `GPIOFRST` reader - GPIOFRST"]
pub type GPIOFRST_R = crate::BitReader<bool>;
#[doc = "Field `GPIOFRST` writer - GPIOFRST"]
pub type GPIOFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O port reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioprstr](index.html) module"]
pub struct IOPRSTR_SPEC;
impl crate::RegisterSpec for IOPRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioprstr::R](R) reader structure"]
impl crate::Readable for IOPRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioprstr::W](W) writer structure"]
impl crate::Writable for IOPRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOPRSTR to value 0"]
impl crate::Resettable for IOPRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
