#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Global interrupt flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CGIF0_AW {
    #[doc = "1: Clear the corresponding CGIFx flag"]
    Clear = 1,
}
impl From<CGIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CGIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGIF0` writer - Global interrupt flag clear for channel x"]
pub type CGIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CGIF0_AW, O>;
impl<'a, const O: u8> CGIF0_W<'a, O> {
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF0_AW::Clear)
    }
}
#[doc = "Transfer complete (TC) flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF0_AW {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    Clear = 1,
}
impl From<CTCIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF0` writer - Transfer complete (TC) flag clear for channel x"]
pub type CTCIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTCIF0_AW, O>;
impl<'a, const O: u8> CTCIF0_W<'a, O> {
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF0_AW::Clear)
    }
}
#[doc = "Half transfer (HT) flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF0_AW {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    Clear = 1,
}
impl From<CHTIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTIF0` writer - Half transfer (HT) flag clear for channel x"]
pub type CHTIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CHTIF0_AW, O>;
impl<'a, const O: u8> CHTIF0_W<'a, O> {
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF0_AW::Clear)
    }
}
#[doc = "Transfer error (TE) flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF0_AW {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    Clear = 1,
}
impl From<CTEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF0` writer - Transfer error (TE) flag clear for channel x"]
pub type CTEIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTEIF0_AW, O>;
impl<'a, const O: u8> CTEIF0_W<'a, O> {
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF0_AW::Clear)
    }
}
#[doc = "Field `CGIF1` writer - Global interrupt flag clear for channel x"]
pub use CGIF0_W as CGIF1_W;
#[doc = "Field `CGIF2` writer - Global interrupt flag clear for channel x"]
pub use CGIF0_W as CGIF2_W;
#[doc = "Field `CGIF3` writer - Global interrupt flag clear for channel x"]
pub use CGIF0_W as CGIF3_W;
#[doc = "Field `CGIF4` writer - Global interrupt flag clear for channel x"]
pub use CGIF0_W as CGIF4_W;
#[doc = "Field `CGIF5` writer - Global interrupt flag clear for channel x"]
pub use CGIF0_W as CGIF5_W;
#[doc = "Field `CGIF6` writer - Global interrupt flag clear for channel x"]
pub use CGIF0_W as CGIF6_W;
#[doc = "Field `CGIF7` writer - Global interrupt flag clear for channel x"]
pub use CGIF0_W as CGIF7_W;
#[doc = "Field `CHTIF1` writer - Half transfer (HT) flag clear for channel x"]
pub use CHTIF0_W as CHTIF1_W;
#[doc = "Field `CHTIF2` writer - Half transfer (HT) flag clear for channel x"]
pub use CHTIF0_W as CHTIF2_W;
#[doc = "Field `CHTIF3` writer - Half transfer (HT) flag clear for channel x"]
pub use CHTIF0_W as CHTIF3_W;
#[doc = "Field `CHTIF4` writer - Half transfer (HT) flag clear for channel x"]
pub use CHTIF0_W as CHTIF4_W;
#[doc = "Field `CHTIF5` writer - Half transfer (HT) flag clear for channel x"]
pub use CHTIF0_W as CHTIF5_W;
#[doc = "Field `CHTIF6` writer - Half transfer (HT) flag clear for channel x"]
pub use CHTIF0_W as CHTIF6_W;
#[doc = "Field `CHTIF7` writer - Half transfer (HT) flag clear for channel x"]
pub use CHTIF0_W as CHTIF7_W;
#[doc = "Field `CTCIF1` writer - Transfer complete (TC) flag clear for channel x"]
pub use CTCIF0_W as CTCIF1_W;
#[doc = "Field `CTCIF2` writer - Transfer complete (TC) flag clear for channel x"]
pub use CTCIF0_W as CTCIF2_W;
#[doc = "Field `CTCIF3` writer - Transfer complete (TC) flag clear for channel x"]
pub use CTCIF0_W as CTCIF3_W;
#[doc = "Field `CTCIF4` writer - Transfer complete (TC) flag clear for channel x"]
pub use CTCIF0_W as CTCIF4_W;
#[doc = "Field `CTCIF5` writer - Transfer complete (TC) flag clear for channel x"]
pub use CTCIF0_W as CTCIF5_W;
#[doc = "Field `CTCIF6` writer - Transfer complete (TC) flag clear for channel x"]
pub use CTCIF0_W as CTCIF6_W;
#[doc = "Field `CTCIF7` writer - Transfer complete (TC) flag clear for channel x"]
pub use CTCIF0_W as CTCIF7_W;
#[doc = "Field `CTEIF1` writer - Transfer error (TE) flag clear for channel x"]
pub use CTEIF0_W as CTEIF1_W;
#[doc = "Field `CTEIF2` writer - Transfer error (TE) flag clear for channel x"]
pub use CTEIF0_W as CTEIF2_W;
#[doc = "Field `CTEIF3` writer - Transfer error (TE) flag clear for channel x"]
pub use CTEIF0_W as CTEIF3_W;
#[doc = "Field `CTEIF4` writer - Transfer error (TE) flag clear for channel x"]
pub use CTEIF0_W as CTEIF4_W;
#[doc = "Field `CTEIF5` writer - Transfer error (TE) flag clear for channel x"]
pub use CTEIF0_W as CTEIF5_W;
#[doc = "Field `CTEIF6` writer - Transfer error (TE) flag clear for channel x"]
pub use CTEIF0_W as CTEIF6_W;
#[doc = "Field `CTEIF7` writer - Transfer error (TE) flag clear for channel x"]
pub use CTEIF0_W as CTEIF7_W;
impl W {
    #[doc = "Bit 0 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif0(&mut self) -> CGIF0_W<0> {
        CGIF0_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif0(&mut self) -> CTCIF0_W<1> {
        CTCIF0_W::new(self)
    }
    #[doc = "Bit 2 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif0(&mut self) -> CHTIF0_W<2> {
        CHTIF0_W::new(self)
    }
    #[doc = "Bit 3 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif0(&mut self) -> CTEIF0_W<3> {
        CTEIF0_W::new(self)
    }
    #[doc = "Bit 4 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<4> {
        CGIF1_W::new(self)
    }
    #[doc = "Bit 5 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<5> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 6 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<6> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 7 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<7> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 8 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<8> {
        CGIF2_W::new(self)
    }
    #[doc = "Bit 9 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<9> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 10 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<10> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 11 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<11> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 12 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<12> {
        CGIF3_W::new(self)
    }
    #[doc = "Bit 13 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<13> {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 14 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<14> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 15 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<15> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 16 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> CGIF4_W<16> {
        CGIF4_W::new(self)
    }
    #[doc = "Bit 17 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<17> {
        CTCIF4_W::new(self)
    }
    #[doc = "Bit 18 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<18> {
        CHTIF4_W::new(self)
    }
    #[doc = "Bit 19 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<19> {
        CTEIF4_W::new(self)
    }
    #[doc = "Bit 20 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> CGIF5_W<20> {
        CGIF5_W::new(self)
    }
    #[doc = "Bit 21 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<21> {
        CTCIF5_W::new(self)
    }
    #[doc = "Bit 22 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<22> {
        CHTIF5_W::new(self)
    }
    #[doc = "Bit 23 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<23> {
        CTEIF5_W::new(self)
    }
    #[doc = "Bit 24 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif6(&mut self) -> CGIF6_W<24> {
        CGIF6_W::new(self)
    }
    #[doc = "Bit 25 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF6_W<25> {
        CTCIF6_W::new(self)
    }
    #[doc = "Bit 26 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF6_W<26> {
        CHTIF6_W::new(self)
    }
    #[doc = "Bit 27 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<27> {
        CTEIF6_W::new(self)
    }
    #[doc = "Bit 28 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cgif7(&mut self) -> CGIF7_W<28> {
        CGIF7_W::new(self)
    }
    #[doc = "Bit 29 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF7_W<29> {
        CTCIF7_W::new(self)
    }
    #[doc = "Bit 30 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF7_W<30> {
        CHTIF7_W::new(self)
    }
    #[doc = "Bit 31 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF7_W<31> {
        CTEIF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
