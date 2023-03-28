#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALRAF` writer - CALRAF"]
pub type CALRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CALRBF` writer - CALRBF"]
pub type CALRBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CWUTF` writer - CWUTF"]
pub type CWUTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CTSF` writer - CTSF"]
pub type CTSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CTSOVF` writer - CTSOVF"]
pub type CTSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITSF` writer - CITSF"]
pub type CITSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CALRAF"]
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CALRAF_W<0> {
        CALRAF_W::new(self)
    }
    #[doc = "Bit 1 - CALRBF"]
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CALRBF_W<1> {
        CALRBF_W::new(self)
    }
    #[doc = "Bit 2 - CWUTF"]
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CWUTF_W<2> {
        CWUTF_W::new(self)
    }
    #[doc = "Bit 3 - CTSF"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<3> {
        CTSF_W::new(self)
    }
    #[doc = "Bit 4 - CTSOVF"]
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CTSOVF_W<4> {
        CTSOVF_W::new(self)
    }
    #[doc = "Bit 5 - CITSF"]
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CITSF_W<5> {
        CITSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
