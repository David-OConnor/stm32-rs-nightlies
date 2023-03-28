#[doc = "Register `FIR1` writer"]
pub struct W(crate::W<FIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIR1_SPEC>;
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
impl From<crate::W<FIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTOHSTX` writer - FTOHSTX"]
pub type FTOHSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FTOLPRX` writer - FTOLPRX"]
pub type FTOLPRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FECCSE` writer - FECCSE"]
pub type FECCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FECCME` writer - FECCME"]
pub type FECCME_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FCRCE` writer - FCRCE"]
pub type FCRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FPSE` writer - FPSE"]
pub type FPSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FEOTPE` writer - FEOTPE"]
pub type FEOTPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FLPWRE` writer - FLPWRE"]
pub type FLPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGCWRE` writer - FGCWRE"]
pub type FGCWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPWRE` writer - FGPWRE"]
pub type FGPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPTXE` writer - FGPTXE"]
pub type FGPTXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPRDE` writer - FGPRDE"]
pub type FGPRDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPRXE` writer - FGPRXE"]
pub type FGPRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - FTOHSTX"]
    #[inline(always)]
    #[must_use]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<0> {
        FTOHSTX_W::new(self)
    }
    #[doc = "Bit 1 - FTOLPRX"]
    #[inline(always)]
    #[must_use]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<1> {
        FTOLPRX_W::new(self)
    }
    #[doc = "Bit 2 - FECCSE"]
    #[inline(always)]
    #[must_use]
    pub fn feccse(&mut self) -> FECCSE_W<2> {
        FECCSE_W::new(self)
    }
    #[doc = "Bit 3 - FECCME"]
    #[inline(always)]
    #[must_use]
    pub fn feccme(&mut self) -> FECCME_W<3> {
        FECCME_W::new(self)
    }
    #[doc = "Bit 4 - FCRCE"]
    #[inline(always)]
    #[must_use]
    pub fn fcrce(&mut self) -> FCRCE_W<4> {
        FCRCE_W::new(self)
    }
    #[doc = "Bit 5 - FPSE"]
    #[inline(always)]
    #[must_use]
    pub fn fpse(&mut self) -> FPSE_W<5> {
        FPSE_W::new(self)
    }
    #[doc = "Bit 6 - FEOTPE"]
    #[inline(always)]
    #[must_use]
    pub fn feotpe(&mut self) -> FEOTPE_W<6> {
        FEOTPE_W::new(self)
    }
    #[doc = "Bit 7 - FLPWRE"]
    #[inline(always)]
    #[must_use]
    pub fn flpwre(&mut self) -> FLPWRE_W<7> {
        FLPWRE_W::new(self)
    }
    #[doc = "Bit 8 - FGCWRE"]
    #[inline(always)]
    #[must_use]
    pub fn fgcwre(&mut self) -> FGCWRE_W<8> {
        FGCWRE_W::new(self)
    }
    #[doc = "Bit 9 - FGPWRE"]
    #[inline(always)]
    #[must_use]
    pub fn fgpwre(&mut self) -> FGPWRE_W<9> {
        FGPWRE_W::new(self)
    }
    #[doc = "Bit 10 - FGPTXE"]
    #[inline(always)]
    #[must_use]
    pub fn fgptxe(&mut self) -> FGPTXE_W<10> {
        FGPTXE_W::new(self)
    }
    #[doc = "Bit 11 - FGPRDE"]
    #[inline(always)]
    #[must_use]
    pub fn fgprde(&mut self) -> FGPRDE_W<11> {
        FGPRDE_W::new(self)
    }
    #[doc = "Bit 12 - FGPRXE"]
    #[inline(always)]
    #[must_use]
    pub fn fgprxe(&mut self) -> FGPRXE_W<12> {
        FGPRXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host force interrupt register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir1](index.html) module"]
pub struct FIR1_SPEC;
impl crate::RegisterSpec for FIR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fir1::W](W) writer structure"]
impl crate::Writable for FIR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIR1 to value 0"]
impl crate::Resettable for FIR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
