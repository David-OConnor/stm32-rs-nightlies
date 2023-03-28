#[doc = "Register `OR2` reader"]
pub struct R(crate::R<OR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR2` writer"]
pub struct W(crate::W<OR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR2_SPEC>;
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
impl From<crate::W<OR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BKINE_R = crate::BitReader<bool>;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BKINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR2_SPEC, bool, O>;
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type BKCMP1E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type BKCMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR2_SPEC, bool, O>;
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type BKCMP2E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type BKCMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR2_SPEC, bool, O>;
#[doc = "Field `BKDF1BK0E` reader - BRK dfsdm1_break\\[0\\]
enable"]
pub type BKDF1BK0E_R = crate::BitReader<bool>;
#[doc = "Field `BKDF1BK0E` writer - BRK dfsdm1_break\\[0\\]
enable"]
pub type BKDF1BK0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR2_SPEC, bool, O>;
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BKINP_R = crate::BitReader<bool>;
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BKINP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR2_SPEC, bool, O>;
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type BKCMP1P_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type BKCMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR2_SPEC, bool, O>;
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity"]
pub type BKCMP2P_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity"]
pub type BKCMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - BRK dfsdm1_break\\[0\\]
enable"]
    #[inline(always)]
    pub fn bkdf1bk0e(&self) -> BKDF1BK0E_R {
        BKDF1BK0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<0> {
        BKINE_W::new(self)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<1> {
        BKCMP1E_W::new(self)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<2> {
        BKCMP2E_W::new(self)
    }
    #[doc = "Bit 8 - BRK dfsdm1_break\\[0\\]
enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkdf1bk0e(&mut self) -> BKDF1BK0E_W<8> {
        BKDF1BK0E_W::new(self)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<9> {
        BKINP_W::new(self)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<10> {
        BKCMP1P_W::new(self)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<11> {
        BKCMP2P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 option register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or2](index.html) module"]
pub struct OR2_SPEC;
impl crate::RegisterSpec for OR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or2::R](R) reader structure"]
impl crate::Readable for OR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or2::W](W) writer structure"]
impl crate::Writable for OR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR2 to value 0"]
impl crate::Resettable for OR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
